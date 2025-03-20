import { PasswordSignalProps } from "./PasswordBox";

export function PasswordField({ password }: Readonly<PasswordSignalProps>) {
  return (
    <label class="w-full">
      <span class="sr-only">Generated password</span>
      <input
        class="w-full input input-xl text-center"
        name="password"
        type="text"
        value={password}
        readonly
      />
    </label>
  );
}
