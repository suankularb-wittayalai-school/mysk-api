// External libraries
import Image from "next/image";
import Head from "next/head";

import { useTranslation } from "next-i18next";
import { serverSideTranslations } from "next-i18next/serverSideTranslations";

// SK Components
import {
  ContentLayout,
  Header,
  MaterialIcon,
  Section,
} from "@suankularb-components/react";

// Types
import { CustomPage, LangCode } from "@/utils/types";
import { useSession } from "next-auth/react";
import useUser from "@/utils/helpers/useUser";
import FetchComponent from "@/components/FetchComponent";

// Page
const IndexPage: CustomPage = () => {
  const { user, accessToken, status } = useUser();

  // console.log(data);

  return (
    <>
      <Head>
        <title>MySK API Test Server</title>
      </Head>
      <ContentLayout>
        <Section>
          {/* <Header>{t("welcome.title")}</Header>
          <p className="skc-body-medium">{t("welcome.desc")}</p> */}
          {status === "loading" && <p>Loading...</p>}
          {status === "unauthenticated" && <p>Not Logged in</p>}
          {status === "authenticated" && <p>Logged in as {user?.email}</p>}
        </Section>
        <FetchComponent accessToken={accessToken ?? undefined} />
      </ContentLayout>
    </>
  );
};

export const getStaticProps = async ({ locale }: { locale: LangCode }) => ({
  props: {
    ...(await serverSideTranslations(locale, ["common"])),
  },
});

IndexPage.pageHeader = {
  title: "SK Shopping Test Server",
  // icon: <MaterialIcon icon="waving_hand" />,
};

export default IndexPage;

