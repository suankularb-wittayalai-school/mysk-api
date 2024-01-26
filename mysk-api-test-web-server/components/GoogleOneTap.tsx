// Imports
import { useOneTapSignin } from "@/utils/helpers/useOneTapSignin";

/**
 * Google’s One Tap Sign in.
 *
 * @returns A hidden `<div>`.
 */
const GoogleOneTap = () => {
  useOneTapSignin({ parentContainerID: "one-tap" });

  return (
    <div
      id="one-tap"
      className="fixed right-0 top-0 dark:right-2 dark:top-2 dark:overflow-hidden dark:rounded-md"
    />
  );
};

export default GoogleOneTap;
