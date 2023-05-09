import { createContext, useContext } from "react";

export enum Accent {
    Sky,
    Indigo
}

export const AccentContext = createContext<Accent>(null);

export const useAccent = () => useContext(AccentContext);
