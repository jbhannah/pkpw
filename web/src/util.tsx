import type { Signal } from "@preact/signals";
import { pkpw } from "pkpw";
import posthog from "posthog-js";

export const generatePassword = (
  password: Signal<string>,
  len: number,
  count: number,
  separator: string,
) => {
  posthog.capture("generated_password");
  password.value = pkpw(len <= 0 ? null : len, count, separator);
};

export const copyPassword = (password: Signal<string>) => {
  posthog.capture("copied_password");
  navigator.clipboard.writeText(password.value);
};
