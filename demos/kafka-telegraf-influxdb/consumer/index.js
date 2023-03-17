import Kafka from 'node-rdkafka';
import avro from 'avsc';

const categories = ['CHANNEL', 'STATE', 'SOURCE']
const eventType = avro.Type.forSchema({
  type: 'record',
  fields: [
    {
      name: 'category',
      type: { type: 'enum', symbols: categories }
    },
    {
      name: 'status',
      type: 'string',
    }
  ]
});

var consumer = new Kafka.KafkaConsumer({
  'group.id': 'kafka',
  'metadata.broker.list': 'broker:29092',
}, {});

consumer.connect();

consumer.on('ready', () => {
  console.log('consumer ready..')
  consumer.subscribe(['settopbox']);
  consumer.consume();
}).on('data', function(data) {
  console.log(`consumer received message: ${eventType.fromBuffer(data.value)}`);
});