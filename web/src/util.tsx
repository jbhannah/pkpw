import type { Signal } from "@preact/signals";
import { pkpw } from "pkpw";
import posthog from "posthog-js";
import { PasswordSeparator } from "./components/PasswordBox";

const DIGITS = "0123456789";
const SPECIAL = "~`!@#$%^&*()_-+={}[]|:;<,>.?/";

const randomSeparatorChar = (): string => {
  const pool = DIGITS.repeat(3) + SPECIAL;
  return pool[Math.floor(Math.random() * pool.length)];
};

export const generatePassword = (
  password: Signal<string>,
  len: number,
  count: number,
  separator: PasswordSeparator,
) => {
  posthog.capture("generated_password", { len, count, separator });
  const effectiveSeparator =
    separator === PasswordSeparator.RANDOM ? randomSeparatorChar() : separator;
  password.value = pkpw(
    len <= 0 ? null : len,
    count <= 0 ? null : count,
    effectiveSeparator,
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
