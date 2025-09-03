import { PasswordSeparator, type PasswordOptionProps } from "./PasswordBox";

const PasswordOptions = ({
  count,
  separator,
}: Readonly<PasswordOptionProps>) => (
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
    <label className="label">
      <span className="label-text">Separator</span>
      <select
        name="separator"
        className="select select-bordered w-full max-w-xs"
        onChange={(event) =>
          (separator.value = event.currentTarget.value as PasswordSeparator)
        }
      >
        {Object.values(PasswordSeparator).map((separatorValue) => (
          <option
            key={separatorValue}
            value={separatorValue}
            selected={separator.value === separatorValue}
          >
            {separatorValue === PasswordSeparator.SPACE && "Space"}
            {separatorValue === PasswordSeparator.DIGITS && "Digits"}
            {separatorValue === PasswordSeparator.SPECIAL &&
              "Special Characters"}
          </option>
        ))}
      </select>
    </label>
  </>
);

export default PasswordOptions;
