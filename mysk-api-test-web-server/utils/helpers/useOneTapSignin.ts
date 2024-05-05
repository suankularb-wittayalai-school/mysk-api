// Imports
import { logError } from "@/utils/helpers/debug";
import { LangCode, User } from "@/utils/types";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { fetchAPI } from "../backend";
import useUser from "./useUser";
import { set } from "date-fns";

type BackendCredentials = {
  access_token: string;
  expires_in: number;
  token_type: string;
  scope: string;
  id_token: string;
};

/**
 * Tap into Google Sign in.
 *
 * @param options.parentContainerId The HTML ID of the One Tap’s container.
 * @param options.parentButtonId The HTML ID of the Sign in Button.
 * @param options.buttonWidth The width of the Sign in Button in pixels.
 *
 * @returns `isLoading`—if One Tap is loading.
 */
export const useOneTapSignin = (options?: {
  parentContainerID?: string;
  parentButtonID?: string;
  buttonWidth?: number;
}) => {
  const { parentContainerID, parentButtonID, buttonWidth } = options || {};
  const [loading, setLoading] = useState(true);

  const router = useRouter();
  const locale = router.locale as LangCode;

  const { user, accessToken, status: UserStatus, error } = useUser();

  // check for access token cookie
  useEffect(() => {
    // console.log({ user, accessToken });

    if (UserStatus !== "loading") {
      console.log({ user, accessToken });
      if (accessToken) {
        console.log("access token found");
        setLoading(false);
        return;
      }

      if (!accessToken) {
        const { google } = window;
        if (!google) return;

        console.log("no access token");

        try {
          google.accounts.id.initialize({
            client_id: process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID!,
            // itp_support: false,
            callback: async (response) => {
              const { data: BackendCredentials, error } =
                await fetchAPI<BackendCredentials>(
                  "/auth/oauth/gsi",
                  undefined,
                  {
                    method: "POST",
                    headers: {
                      "Content-Type": "application/json",
                    },
                    body: JSON.stringify({ credential: response.credential }),
                  }
                );
              console.log({ response });

              if (error) {
                // throw new Error(error.detail);
                logError("useOneTapSignin", error);
                setLoading(false);
                return;
              }

              console.log({ BackendCredentials, error });

              const { data: user, error: userError } = await fetchAPI<User>(
                "/auth/user",
                undefined,
                {},
                BackendCredentials.access_token
              );

              if (userError) {
                // throw new Error(userError.detail);
                logError("useOneTapSignin", userError);
                setLoading(false);
                return;
              }

              // console.log(user);
              // set cookies for session
              if (user) {
                document.cookie = `access_token=${
                  BackendCredentials.access_token
                }; path=/; expires=${new Date(
                  Date.now() + BackendCredentials.expires_in * 1000
                ).toUTCString()}`;
              }

              router.push("/");
              setLoading(false);
            },
            prompt_parent_id: parentContainerID,
          });

          // Render Google One Tap on supported browsers once the parent
          // container is rendered
          if (parentContainerID && document.getElementById(parentContainerID))
            google.accounts.id.prompt((notification) => {
              if (notification.isNotDisplayed())
                logError("useOneTapSignin", {
                  error_type: "getNotDisplayedReason",
                  detail: notification.getNotDisplayedReason(),
                });
              else if (notification.isSkippedMoment())
                logError("useOneTapSignin", {
                  error_type: "getNotDisplayedReason",
                  detail: notification.getSkippedReason(),
                });
              else if (notification.isDismissedMoment())
                logError("useOneTapSignin", {
                  error_type: "getDismissedReason",
                  detail: notification.getDismissedReason(),
                });
            });

          // Render the Sign in button if provided with an ID
          if (parentButtonID) {
            google.accounts.id.renderButton(
              document.getElementById(parentButtonID) as HTMLElement,
              {
                shape: "pill",
                text: "continue_with",
                width: buttonWidth,
                locale,
              }
            );
          }
        } catch (error) {
          logError("useOneTapSignin", {
            error_type: "googleOneTap",
            detail: error as string,
          });
        }
      }
    }
  }, [UserStatus, accessToken, locale, parentButtonID, parentContainerID]);

  return { loading };
};

