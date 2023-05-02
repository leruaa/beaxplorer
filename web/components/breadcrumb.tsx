import { CaretRight, House, Icon, IconContext } from "@phosphor-icons/react";

type Props = { children?: JSX.Element | JSX.Element[] }

export const Part = ({ children }: Props) => {
  return (
    <>
      <CaretRight className="inline mb-1 text-gray-500" />
      {children}
    </>
  );
}

export const Root = ({ children }: Props) => {
  return (
    <IconContext.Provider
      value={{
        size: "1em",
        weight: "bold",
        className: "inline mb-1",
      }}
    >
      <h2 className="container mx-auto">
        <ul className="breadcrumb">
          <a href="/"><House /></a>
          {children}
        </ul>
      </h2>
    </IconContext.Provider>
  );
}