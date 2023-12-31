const { fontFamily } = require('tailwindcss/defaultTheme');
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './templates/*.html',
    './templates/**/*.html'
  ],
  theme: {
      extend: {
          fontFamily: {
              sans: ['Inter var', ...fontFamily.sans],
          },
      },
  },
  plugins: [require('@tailwindcss/forms'), require('@tailwindcss/typography')],
};

