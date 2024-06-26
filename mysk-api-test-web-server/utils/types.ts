// External libraries
import { NextPage } from "next";
import { AppProps } from "next/app";

// SK Components
import { PageHeaderProps } from "@suankularb-components/react";

/**
 * The language code of a supported UI language.
 */
export type LangCode = "en-US" | "th";

/**
 * The {@link NextPage} type extended with properties for SKCom.
 */
export type CustomPage = NextPage & {
  /**
   * A FAB to place in Navigation Rail for or fixed on this page only.
   *
   * @see {@link https://docs.google.com/document/d/1UJeTpXcB2MBL9Df4GUUeZ78xb-RshNIC_-LCIKmCo-8/edit?usp=sharing#heading=h.v2ft1p7l7f8a SKCom documentation on FAB}
   */
  fab?: JSX.Element;

  /**
   * Additional props for the Page Header component, applied specifically to
   * this page only.
   *
   * `title` is required.
   *
   * @see {@link https://docs.google.com/document/d/1UJeTpXcB2MBL9Df4GUUeZ78xb-RshNIC_-LCIKmCo-8/edit?usp=sharing#heading=h.5w06ou3fwzsd SKCom documentation on Page Header}
   */
  pageHeader?: {
    /**
     * The title text: the biggest text in a page and the only within a `<h1>`
     * tag.
     *
     * - You can use next-i18next by passing in the key and the namespace in an
     *   object, i.e. `{ key: "title", ns: "home" }`
     * - Always required.
     */
    title: string | JSX.Element | { key: string; ns: string };
  } & Partial<Omit<PageHeaderProps, "title">>;

  /**
   * A list of child URLs of the current page.
   */
  childURLs?: string[];
};

/**
 * The {@link AppProps} type extended with properties for SKCom.
 */
export type CustomAppProps = {
  Component: CustomPage;
  pageProps: AppProps["pageProps"];
};

/**
 * An error object returned from the Club Registry API.
 */
export type APIError = {
  id: string;
  code: number;
  error_type: string;
  detail: string;
  source: string;
};

/**
 * A string that supports Thai and English, with the latter being optional.
 */
export type MultiLangString = {
  th: string;
  "en-US"?: string;
};

export enum UserRole {
  student = "student",
  teacher = "teacher",
  management = "management",
  organization = "organization",
  staff = "staff",
}

/**
 * The key of a User Permission.
 */
export enum UserPermissionKey {
  /**
   * Can see the Manage page and its children.
   */
  can_see_management = "can_see_management",
}

export type User = {
  id: string;
  email: string | null;
  permissions: UserPermissionKey[];
  is_admin: boolean;
  onboarded: boolean;
  role: UserRole;
};

