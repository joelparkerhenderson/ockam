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

const stream = Kafka.Producer.createWriteStream({
  'metadata.broker.list': 'broker:29092'
}, {}, {
  topic: 'settopbox'
});

stream.on('error', (err) => {
  console.error('Error in our kafka stream');
  console.error(err);
});

function queueRandomMessage() {
  const category = getRandomCategory();
  const status = getRandomData(category);
  const event = { category, status };
  const success = stream.write(eventType.toBuffer(event));     
  if (success) {
    console.log(`message queued (${JSON.stringify(event)})`);
  } else {
    console.log('Too many messages in the queue already..');
  }
}

function getRandomCategory() {
  return categories[Math.floor(Math.random() * categories.length)];
}

function getRandomData(category) {
  let statuses
  if (category === 'CHANNEL') {
    statuses = ['NBC', 'CNN', 'CBS', 'FOX'];
  } else if (category === 'STATE') {
    statuses = ['PLAY', 'OFF', 'PAUSE'];
  } else if (category === 'SOURCE') {
    statuses = ['DTS', 'HDMI1','SMARTTV', 'HDMI2', 'AV1'];
  } else {
    statuses = ['UNKNOWN'];
  }
  return statuses[Math.floor(Math.random() * statuses.length)];
}

setInterval(() => {
  queueRandomMessage();
}, 3000);