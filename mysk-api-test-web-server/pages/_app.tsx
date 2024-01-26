// External libraries
import { MotionConfig } from "framer-motion";

import {
  Fira_Code,
  IBM_Plex_Sans_Thai,
  Inter,
  Sarabun,
  Space_Grotesk,
} from "next/font/google";
import localFont from "next/font/local";

import { appWithTranslation } from "next-i18next";

import { FC, ReactNode, useState } from "react";

// SK Components
import { ThemeProvider } from "@suankularb-components/react";

// Internal components
import Layout from "@/components/Layout";

// Contexts
import SnackbarContext from "@/contexts/SnackbarContext";

// Styles
import "@/styles/globals.css";

// Utilities
import { CustomAppProps } from "@/utils/types";

// English fonts
const bodyFontEN = Inter({ subsets: ["latin"] });
const displayFontEN = Space_Grotesk({ subsets: ["latin"] });

// Thai fonts
const bodyFontTH = Sarabun({
  weight: ["300", "400", "500", "700"],
  subsets: ["thai"],
});
const displayFontTH = IBM_Plex_Sans_Thai({
  weight: ["300", "400", "500", "700"],
  subsets: ["thai"],
});

// Mono font
const monoFont = Fira_Code({ subsets: ["latin"] });

// Icon font
const iconFont = localFont({
  src: "../public/fonts/material-symbols.woff2",
  weight: "100 700",
  style: "normal",
});

/**
 * To prevent the App component from being more of a triangle than it already
 * is, all the context providers are extracted into this component.
 *
 * @param children The app that uses contexts.
 *
 * @returns The app wrapped with context providers.
 */
const Contexts: FC<{ children: ReactNode }> = ({ children }) => {
  const [snackbar, setSnackbar] = useState<JSX.Element | null>(null);

  return (
    <SnackbarContext.Provider value={{ snackbar, setSnackbar }}>
      {/* Add more contexts here */}
      {children}
    </SnackbarContext.Provider>
  );
};

function App({ Component, pageProps: { ...pageProps } }: CustomAppProps) {
  const { fab, pageHeader, childURLs } = Component;

  return (
    <>
      <style jsx global>{`
        :root {
          --font-body: -apple-system, BlinkMacSystemFont,
            ${bodyFontEN.style.fontFamily}, ${bodyFontTH.style.fontFamily};
          --font-display: ${displayFontEN.style.fontFamily},
            ${displayFontTH.style.fontFamily};
          --font-mono: ui-monospace, SFMono-Regular, SF Mono,
            ${monoFont.style.fontFamily}, ${bodyFontTH.style.fontFamily};
          --font-icon: ${iconFont.style.fontFamily};
        }
      `}</style>

      <Contexts>
        <MotionConfig reducedMotion="user">
          <ThemeProvider>
            <Layout {...{ fab, pageHeader, childURLs }}>
              <Component {...pageProps} />
            </Layout>
          </ThemeProvider>
        </MotionConfig>
      </Contexts>
    </>
  );
}

export default appWithTranslation(App);
