import { createContext, useContext } from "react";

export enum Accent {
    Green,
    Yellow,
    Sky,
    Indigo,
    Purple
}

export const AccentContext = createContext<Accent>(null);

export const useAccent = () => useContext(AccentContext);
