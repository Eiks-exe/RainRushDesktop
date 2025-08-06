import React from 'react';
import authStyle from './auth.module.css'; // Assuming you have a CSS module for styling

interface AuthLayoutProps {
  children?: React.ReactNode;
}

const AuthLayout: React.FC<AuthLayoutProps>= ({children}) => {
  
  return (
    <div className={authStyle.auth_wrapper}>{children}</div>
  );
};

export default AuthLayout;

