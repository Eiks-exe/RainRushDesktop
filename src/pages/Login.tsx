import DiscordIcon from '../utils/DiscordIcon';
import AuthLayout from '../components/AuthLayout/AuthLayout';
import authstyles from '../components/AuthLayout/auth.module.css';
import { useAppContext } from '../contexts/AppContext';
import { useAuth } from '../contexts/AuthContext';
import React from 'react';
import { login_back } from '../utils/fetchData';

const Login = () => {
  const {setView} = useAppContext();
  const { loginUser } = useAuth();
  const identifierRef = React.useRef<HTMLInputElement>(null);
  const passwordRef = React.useRef<HTMLInputElement>(null);
  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    const identifier = identifierRef.current?.value; 
    const password = passwordRef.current?.value;
    if(!identifier || !password) {
      return;
    }

    await loginUser(identifier, password);
  }  

  return(
    <AuthLayout>
       <div className={authstyles.auth_container}>
        <h1 className={authstyles.auth_title}>Login</h1>
        <button className={authstyles.discord_button}>
          <DiscordIcon /> Log in with discord
        </button>
        <div className={authstyles.separator} />
        <form onSubmit = {(e) => {handleLogin(e)} } className={authstyles.form_container}>
          <input
            ref={identifierRef}
            type="text"
            placeholder="Email"
            className={authstyles.form_input}
          />
          <input
            ref= {passwordRef}
            type="password"
            placeholder="Password"
            className={authstyles.form_input}
          />
          <div className={authstyles.radio_container}>
            <input type="checkbox" className={authstyles.form_checkbox} />
            <label className={authstyles.form_label}>Remember me</label>
          </div>
          <a className={authstyles.auth_link}>
            Forgot password?
          </a>
          
          <button type="submit" className={authstyles.form_button}>
            Login
          </button>

        </form>
        <div className={authstyles.separator} />
        <p className={authstyles.auth_text}>
          Don't have an account? 
          <button className={authstyles.auth_link} onClick={()=>{setView('register')}}>Sign up</button>
        </p>
      </div>
    </AuthLayout>
  )
}

export default Login;

