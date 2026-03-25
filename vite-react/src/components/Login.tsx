import { useState } from "react";
import { useNavigate } from 'react-router-dom';
import Mfa from "./Mfa.tsx";
import jQuery from "jquery";
import { useMutation as useApolloMutation } from '@apollo/client/react';
import { SIGNIN_MUTATION } from "../graphql/login_query.ts";
import type { LoginUserData, LoginUserVariables } from "../graphql/login_query.ts";

export default function Login() {
   const [username, setUsername] = useState<string>('');
   const [password, setPassword] = useState<string>('')
   const [message, setMessage] = useState<string>('');
   const [isdiabled, setIsdisabled] = useState(false);
   const navigate = useNavigate();

  const [signinUser ] = useApolloMutation<LoginUserData, LoginUserVariables>(SIGNIN_MUTATION, {
      onCompleted: (data: any) => {
        setMessage(data.signinUser.message);
        let userpic: string = `http://127.0.0.1:3000/assets/users/${data.signinUser.userpic}`
        if (data.signinUser.qrcodeurl !== null) {
          window.sessionStorage.setItem('USERID',data.signinUser.id);
          window.sessionStorage.setItem('TOKEN',data.signinUser.token);
          window.sessionStorage.setItem('USERPIC', userpic);
          jQuery("#loginReset").trigger("click");
          setIsdisabled(false);
          jQuery("#mfaModal").trigger("click");
        } else {
          window.sessionStorage.setItem('USERID',data.signinUser.id);
          window.sessionStorage.setItem('USERNAME',data.signinUser.username);
          window.sessionStorage.setItem('TOKEN',data.signinUser.token);                        
          window.sessionStorage.setItem('USERPIC', userpic);
          setIsdisabled(false);
          jQuery("#loginReset").trigger('"click')
          closeLogin;
          navigate('/'); 
          location.reload();
        }
        setTimeout(() => { setMessage(''); }, 3000);
      },
      onError: (err: any) => {
        setMessage(err.message);      
        setTimeout(() => { setMessage(''); setIsdisabled(false); }, 3000);
      }
  });

   const submitLogin = async (event: React.SubmitEvent) => {
    event.preventDefault();
    setMessage('please wait...');
    setIsdisabled(true);
    try {
        await signinUser({
          variables: {
            input: {
            username: username,
            password: password
            }
          }
        });
      } catch (err: any) {
        setMessage(err.message);
        setTimeout(() => { setMessage(''); setIsdisabled(false); }, 3000);
      }
  }

  const closeLogin = (event: any) => {
    event.preventDefault();
    setIsdisabled(false);    
    setMessage('');
    setUsername('');
    setPassword('');
  }

  return (
    <>
<div className="modal fade" id="staticLogin" data-bs-backdrop="static" data-bs-keyboard="false" tabIndex={-1} aria-labelledby="staticLoginLabel" aria-hidden="true">
  <div className="modal-dialog modal-sm modal-dialog-centered">
    <div className="modal-content">
      <div className="modal-header bg-violet">
        <h1 className="modal-title text-white fs-5" id="staticLoginLabel">User's Login</h1>
        <button onClick={closeLogin} type="button" className="btn-close btn-close-white" data-bs-dismiss="modal" aria-label="Close"></button>
      </div>
      <div className="modal-body">
        <form onSubmit={submitLogin} autoComplete="off">
        <div className="mb-3">
          <input type="text" required value={username} onChange={e => setUsername(e.target.value)} className="form-control border-secondary border-emboss" disabled={isdiabled} autoComplete='off' placeholder="enter Username"/>
        </div>          
        <div className="mb-3">
          <input type="password" required value={password} onChange={e => setPassword(e.target.value)} className="form-control border-secondary border-emboss" disabled={isdiabled} autoComplete='off' placeholder="enter Password"/>
        </div>          
        <div className="mb-3">
          <button type="submit" className="btn btn-violet text-white mx-2" disabled={isdiabled}>login</button>
          <button id="loginReset" onClick={closeLogin} type="reset" className="btn btn-violet text-white">reset</button>
          <button id="mfaModal" type="button" className="btn btn-warning d-none" data-bs-toggle="modal" data-bs-target="#staticMfa">mfa</button>

          </div>
        </form>
      </div>
      <div className="modal-footer">
        <div className="w-100 text-danger">{message}</div>
      </div>
    </div>
  </div>
</div>    
<Mfa/>
</>
  )
}
