import type { PasswordSignalProps } from "./PasswordBox";

const PasswordField = ({ password }: Readonly<PasswordSignalProps>) => (
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

export default PasswordField;
