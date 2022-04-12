const api = require('./api')

const buildApi = (options) => {
  return api(options)
}

module.exports = { buildApi }
