import { createContext, useContext, useState } from 'react';
import { IUser } from '../interfaces/IUser';

interface AppContextType {
  user: IUser | null;
  setUser: (user: IUser | null) => void;
  view: string;
  setView: (view: string) => void;
}

const initialState: AppContextType = {
  user: null,
  setUser: () => {}, 
  view: 'login',
  setView: () => {}
};

export const AppContext = createContext<AppContextType>(initialState);

export const AppContextProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<IUser | null>(null);
  const [view, setView] = useState<string>('login');

  return (
    <AppContext.Provider value={{ user, setUser, view, setView }}>
      {children}
    </AppContext.Provider>
  );
}

export const useAppContext = () => {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error('useAppContext must be used within an AppContextProvider');
  }
 return context;
}


