FROM node:lts-alpine3.17
RUN apk add --update --no-cache python3 g++ make bash
COPY ./* ./
RUN npm install
CMD [ "node", "index.js" ]