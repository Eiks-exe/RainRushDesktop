import React from 'react';

import AuthLayout from '../components/AuthLayout/AuthLayout';
import DiscordIcon from '../utils/DiscordIcon';
import authstyles from '../components/AuthLayout/auth.module.css';
import { useAppContext } from '../contexts/AppContext';

const Register: React.FC = () => {
  const handleRegister = (e: React.MouseEvent) => {
    e.preventDefault();
    console.log("Register button clicked");
  }
  const {setView} = useAppContext();

  return(
    <AuthLayout>
       <div className={authstyles.auth_container}>
        <h1 className={authstyles.auth_title}>Register</h1>
        <button className={authstyles.discord_button}>
          <DiscordIcon /> Log in with discord
        </button>
        <div className={authstyles.separator} />
        <form>
          <input
            type="text"
            placeholder="Email"
            className={authstyles.form_input}
          />
          <input
            type="password"
            placeholder="Password"
            className={authstyles.form_input}
          />
          <input
            type="password"
            placeholder="Confirm Password"
            className={authstyles.form_input}
          />
          <div className={authstyles.radio_container}>
            <input type="checkbox" className={authstyles.form_checkbox} />
            <label className={authstyles.form_label}>Remember me</label>
          </div>
          <a className={authstyles.auth_link}>
            Forgot password?
          </a>
          
          <button type="submit" className={authstyles.form_button} onClick={(e : React.MouseEvent) => {
            handleRegister(e);
          }}>
            Signup
          </button>

        </form>
        <div className={authstyles.separator} />
        <p className={authstyles.auth_text}>
          Already have an account?  
          <button className={authstyles.auth_link} onClick={()=>{setView('login')}}>Log in</button>
        </p>
      </div>
    </AuthLayout>
  )
}

export default Register;
