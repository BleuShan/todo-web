module.exports = {
  syntax: 'postcss-scss',
  plugins: {
    cssnano: {
      preset: 'advanced'
    },
    'postcss-font-magician': {
      display: 'swap',
      foundries: 'google'
    }
  }
}
