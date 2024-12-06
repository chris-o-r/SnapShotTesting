import React, { createContext, useContext, useState } from "react";


export const PageTitleContext = createContext<{title: string, setTitle: React.Dispatch<React.SetStateAction<string>>}>(null!);

type Props = {
  children: React.ReactElement;
};
export const PageTitleProvider = ({ children }: Props) => {
  const [title, setTitle] = useState("Snap Shot Testing");

  return (
    <PageTitleContext.Provider
      value={{
        title,
        setTitle,
      }}
    >
      {children}
    </PageTitleContext.Provider>
  );
};

export const usePageTitle = () => {
  return useContext(PageTitleContext);
};
