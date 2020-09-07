import cssnano from 'cssnano'
import fontMagician from 'postcss-font-magician'
import preset from 'postcss-preset-env'

export default {
  syntax: 'postcss-scss',
  plugins: [
    preset({
      stage: 0
    }),
    cssnano({
      preset: 'advanced'
    }),
    fontMagician({
      display: 'swap',
      variants: {
        Roboto: {
          300: [],
          400: [],
          700: []
        }
      },
      foundries: 'google'
    })
  ]
}
