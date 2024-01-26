// a hook that get access token from cookie and fetch user data from backend
// if there is no access token, it will return null

import { APIError, User } from "@/utils/types";
import { useEffect, useState } from "react";
import { fetchAPI } from "@/utils/backend";

function getCookie(name: string): string | null {
  const nameLenPlus = name.length + 1;
  return (
    document.cookie
      .split(";")
      .map((c) => c.trim())
      .filter((cookie) => {
        return cookie.substring(0, nameLenPlus) === `${name}=`;
      })
      .map((cookie) => {
        return decodeURIComponent(cookie.substring(nameLenPlus));
      })[0] || null
  );
}

// if there is access token, it will fetch user data from backend
export default function useUser() {
  const [user, setUser] = useState<User | null>(null);
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [status, setStatus] = useState<
    "unauthenticated" | "authenticated" | "loading"
  >("loading"); // ["idle", "loading", "error"
  const [error, setError] = useState<APIError | null>(null);

  useEffect(() => {
    // const cookies = document.cookie;
    // const accessToken = cookies
    //   .split(";")
    //   .find((cookie) => {
    //     const [key, value] = cookie.split("=");
    //     return key === "access_token";
    //   })
    //   ?.split("=")[1];

    const accessToken = getCookie("access_token");

    if (!accessToken) {
      //   console.log("no access token");
      setUser(null);
      //   setLoading(false);
      setStatus("unauthenticated");
      return;
    }

    (async () => {
      const { data: user, error } = await fetchAPI<User>(
        "/auth/user",
        undefined,
        {},
        accessToken
      );
      if (error) {
        setUser(null);
        setError(error);
        // setLoading(false);
        setStatus("unauthenticated");
        return;
      }

      setAccessToken(accessToken);
      setUser(user);
      setStatus("authenticated");
    })();

    // setLoading(false);
  }, []);

  return { user, accessToken, status, error };
}
