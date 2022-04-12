const pino = require('pino')

const mkLogger = (logLevel) => {
  const logger = pino(
    {
      name: 'API',
      level: logLevel,
      redact: {
        paths: ['USER_URI', '[*].USER_URI'],
        censor: (args) => {
          if (args === '' || args === null || args === undefined) {
            return '[EMPTY]'
          } else {
            return '[REDACTED]'
          }
        },
      },
    },
    process.stdout
  )
  return logger
}
module.exports = mkLogger
