# Suankularb Next.js Template

This repository is a template for all Suankularb websites using the Next.js framework. It includes SK Components, Next.js, Tailwind CSS, Framer Motion, and next-18next internalization solution all configured and ready to use out of the box.

If you know Next.js, you already know enough to get started. We look forward to see what you’re going to create!

> **Note**: If you are making a Suankularb website, don’t clone this repository directly. Use the “[Use this template](https://github.com/suankularb-wittayalai-school/sk-nextjs-template/generate)” button.

## Setup

After cloning, run this command at the root of the repository to install packages.

```
npm i
```

### Prettier

This template uses Prettier to format files. Install Prettier on your code editor and ensure it is the default formatter. Prettier should detect the configuration file `.prettierrc.json`.

## Development

Run this command to start the development server. There’s no need to run any additional commands for compiling Tailwind CSS.

```
npm run dev
```

### Imports

This template uses non-relative module imports. Start an internal import with `@/`.

Refrain from using relative imports.

### SK Components

This template uses ReSKCom: the official React implementation of [SK Components](https://docs.google.com/document/d/1ks5DrzfC_xLg48EFtZALoVQpJpxhsK2It3GDhAhZCcE).

#### Snackbars

This template comes with a [Snackbar](https://docs.google.com/document/d/1ks5DrzfC_xLg48EFtZALoVQpJpxhsK2It3GDhAhZCcE/edit?usp=sharing#heading=h.8mcuhjpzo8bj) management solution. To show a Snackbar, simply use the `setSnackbar` function from the `SnackbarContext` context, like so:

```tsx
const { setSnackbar } = useContext(SnackbarContext);

// Call `setSnackbar` when you want to show a Snackbar:
setSnackbar(<Snackbar>Task failed successfully</Snackbar>);
```

### Framer Motion

This template uses Framer Motion for animation. When using animation in a page or component, make sure to use the `transition` function and the `useAnimationConfig` hook. You can import those from `@suankularb-components/react`.

An example of using Framer Motion with `transition` and `useAnimationConfig`:

```tsx
const Example: FC = () => {
  const { duration, easing } = useAnimationConfig();

  return (
    <motion.p
      initial={{ rotate: 180 }}
      animate={{ rotate: 0 }}
      transition={transition(duration.short4, easing.standard)}
    >
      Hello Framer!
    </motion.p>
  );
};
```

### Tailwind CSS

This template uses Tailwind CSS. To add styles, add utility classes directly to your page or component. Try not to add to `./src/styles/global.css`.

You can find all SKCom tokens in the `tailwind.config.js` file. These can be used in place of the default preset tokens; for example, `bg-primary` sets the background to the default primary color of SKCom (`#00658f`, or `#80cfff` in dark mode).

### Internationalization

This template, by default, supports American English (`en-US`) and Thai (`th`). To add a language, update `./next-i18next.config.js`, update `LangCode` in `./src/utils/types/common.ts`, and add a folder in `./public/static/locales` for your translation files.

Some components in ReSKCom have parts that need translation. You can pass the language via the `locale` prop.

## Production

Build your Suankularb website with this command.

```
npm run build
```

Then start the production server.

```
npm start
```
