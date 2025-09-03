import type { PasswordOptionProps } from "./PasswordBox";

const PasswordOptions = ({ count }: Readonly<PasswordOptionProps>) => (
  <>
    <label className="label">
      <span className="label-text">Count</span>
      <input
        className="input input-bordered"
        type="number"
        value={count}
        min="0"
        onInput={(event) => (count.value = parseInt(event.currentTarget.value))}
      />
    </label>
  </>
);

export default PasswordOptions;
