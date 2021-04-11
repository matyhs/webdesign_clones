module.exports = {
  prefix: 'tw-',
  purge: {
    enabled: !process.env.ROLLUP_WATCH,
    content: ['./public/index.html', './src/**/*.svelte'],
    options: {
      defaultExtractor: content => [
        ...(content.match(/[^<>"'`\s]*[^<>"'`\s:]/g) || []),
        ...(content.match(/(?<=class:)[^=>\/\s]*/g) || [])
      ]
    }
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      height: {
        'screen80': '80vh'
      },
      flexGrow: {
        '3': 3
      },
      borderRadius: {
        '5xl': '3.5rem'
      },
      strokeWidth: {
        '1/2': '0.5',
        '3': '3'
      }
    },
  },
  variants: {
    extend: {
      flexGrow: ['hover']
    },
  },
  plugins: [],
}
