/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./index.html', './src/components/**/*.{vue,js,ts}', './src/pages/**/*.{vue,js,ts}'],
  theme: {
    screens: {
      sm: '640px',
      md: '768px',
      lg: '1024px',
      xl: '1440px',
      hd: '1920px',
    },

    colors: {
      primary: '#F9FF73',
      secondary: '#78DCE8',

      transparent: 'transparent',
      current: 'currentColor',
      white: '#F0F2DA',
      black: '#000000',
      yellow: '#F9FF73',
      orange: '#F7AF39',
      pink: '#FF6188',
      red: '#FF0000',
      green: '#A9DC76',
      violet: '#AB9DF2',
      blue: '#78DCE8',

      body: '#9D9E91',
      bodyDark: '#6A6B63',

      bg: {
        lighter: '#313442',
        light: '#1E212B',
        DEFAULT: '#141721',
        dark: '#06080F',
      },
    },

    fontFamily: {
      sans: ['IBM Plex Sans', 'ui-sans-serif', 'system-ui', 'sans-serif'],
      mono: ['IBM Plex Mono', 'ui-sans-serif', 'system-ui', 'sans-serif'],
      heading: ['Cabinet Grotesk', 'Arial', 'Helvetica', 'sans-serif'],
    },

    container: {
      center: true,
      screens: ['440px'],
      padding: {
        DEFAULT: '20px',
      },
    },

    extend: {
      backgroundImage: {
        gradientDark: 'linear-gradient(180deg, #06080F 0%, rgba(6, 8, 15, 0) 100%)',
        gradientDarkReverse: 'linear-gradient(180deg, rgba(6, 8, 15, 0) 0%, #06080F 100%)',
      },
      borderWidth: {
        1: '1px',
        3: '3px',
      },
      boxShadow: {
        black: '0px 2px 4px rgba(0, 0, 0, 0.12)',
      },
      zIndex: {
        1: 1,
        2: 2,
        3: 3,
        4: 4,
        5: 5,
        6: 6,
        7: 7,
        8: 8,
        9: 9,
      },
    },
  },
  plugins: [],
};
