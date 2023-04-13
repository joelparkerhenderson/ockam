use crate::identity::Identity;
use crate::secure_channel::addresses::Addresses;
use crate::secure_channel::common::{CreateResponderChannelMessage, Role};
use crate::secure_channel::options::SecureChannelListenerOptions;
use crate::secure_channel::DecryptorWorker;
use crate::secure_channels::secure_channels::SecureChannels;
use ockam_core::compat::boxed::Box;
use ockam_core::compat::sync::Arc;
use ockam_core::{Address, AllowAll, DenyAll, Result, Routed, Worker};
use ockam_node::Context;

pub(crate) struct IdentityChannelListener {
    secure_channels: Arc<SecureChannels>,
    identity: Identity,
    options: SecureChannelListenerOptions,
}

impl IdentityChannelListener {
    pub fn new(
        secure_channels: Arc<SecureChannels>,
        identity: Identity,
        options: SecureChannelListenerOptions,
    ) -> Self {
        Self {
            secure_channels,
            identity,
            options,
        }
    }

    pub async fn create(
        ctx: &Context,
        secure_channels: Arc<SecureChannels>,
        identity: &Identity,
        address: Address,
        options: SecureChannelListenerOptions,
    ) -> Result<()> {
        options.setup_flow_control_for_listener(ctx.flow_controls(), &address);

        let listener = Self::new(secure_channels.clone(), identity.clone(), options);

        ctx.start_worker(
            address, listener, AllowAll, // TODO: @ac allow to customize
            DenyAll,
        )
        .await?;

        Ok(())
    }
}

#[ockam_core::worker]
impl Worker for IdentityChannelListener {
    type Message = CreateResponderChannelMessage;
    type Context = Context;

    async fn handle_message(
        &mut self,
        ctx: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let addresses = Addresses::generate(Role::Responder);
        let flow_control_id = self.options.setup_flow_control_for_channel(
            ctx.flow_controls(),
            &addresses,
            &msg.src_addr(),
        );
        let access_control = self
            .options
            .create_access_control(ctx.flow_controls(), flow_control_id);

        DecryptorWorker::create_responder(
            ctx,
            self.secure_channels.clone(),
            addresses,
            self.identity.clone(),
            self.options.trust_policy.clone(),
            access_control.decryptor_outgoing_access_control,
            msg,
        )
        .await
    }
}
