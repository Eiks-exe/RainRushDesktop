import { createContext, useContext, useReducer } from 'react';
import { IUser } from '../interfaces/IUser';
import { checkApi, login_back } from '../utils/fetchData';
import React from 'react';

interface AuthState {
  user: IUser | null;
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

interface AuthAction {
  type: string;
  payload?: any;
}

interface AuthContextType {
  state: AuthState;
  dispatch: React.Dispatch<AuthAction>;
}

const initialUserState: AuthState = {
  user: null,
  token: null,
  isAuthenticated: false,
  isLoading: false,
}

const authReducer = (state: AuthState, action: AuthAction ) => {
  switch (action.type) {
    case 'LOGIN':
      return {
        ...state,
        user: action.payload.user,
        token: action.payload.token,
        isAuthenticated: true,
        isLoading: false,
      };
    case 'LOGOUT':
      return initialUserState;
    case 'LOADING':
      return { ...state,
        isLoading: true,
      };
    default: return state; }
}

export const AuthContext = createContext<AuthContextType| undefined>(undefined);

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [state, dispatch] = useReducer(authReducer, initialUserState);


  return (
    <AuthContext.Provider value={{state, dispatch}}>
      {children}
    </AuthContext.Provider>
  );
}

export const useAuthContext = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuthContext must be used within an AuthProvider');
  }
  return context;
}

export const useAuth = () => {
  const { state, dispatch } = useAuthContext();

  const loginUser = async (identifier: string, password: string) => {
    dispatch({ type: 'LOADING' });
    try {
      const response = await login_back(identifier, password);
      if (response) {
        const { id, username, email, token } = response;
        dispatch({ type: 'LOGIN', payload: { user: {id, name: username, email} , token: token } });
      }
    } catch (error) {
      console.error('Login error:', error);
    }
  }; 

  const logoutUser = () => { dispatch({ type: 'LOGOUT' }); }; return { state, loginUser, logoutUser }; }
