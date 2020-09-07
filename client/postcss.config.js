module.exports = {
  syntax: 'postcss-scss',
  plugins: {
    'postcss-preset-env': {
      stage: 0
    },
    cssnano: {
      preset: 'advanced'
    },
    'postcss-font-magician': {
      display: 'swap',
      variants: {
        Roboto: {
          300: [],
          400: [],
          700: []
        }
      },
      foundries: 'google'
    }
  }
}
