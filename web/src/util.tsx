import type { Signal } from "@preact/signals";
import { pkpw } from "pkpw";
import posthog from "posthog-js";
import type { PasswordSeparator } from "./components/PasswordBox";

export const generatePassword = (
  password: Signal<string>,
  len: number,
  count: number,
  separator: PasswordSeparator,
) => {
  posthog.capture("generated_password", { len, count, separator });
  password.value = pkpw(
    len <= 0 ? null : len,
    count <= 0 ? null : count,
    separator,
  );
};

export const copyPassword = (password: string) => {
  posthog.capture("copied_password");
  navigator.clipboard.writeText(password);
};

export const toggleOptions = (optionsVisible: Signal<boolean>) => {
  posthog.capture(optionsVisible.value ? "showed_options" : "hid_options");
  optionsVisible.value = !optionsVisible.value;
};
