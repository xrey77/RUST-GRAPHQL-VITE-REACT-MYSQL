import { gql } from '@apollo/client';

export const SIGNIN_MUTATION = gql`
  mutation SigninUser($input: SigninInput!) {
    signinUser(input: $input) {
      id
      firstname
      lastname
      email
      mobile
      username
      userpic
      isactivated
      isblocked
      mailtoken
      userpic
      qrcodeurl
      rolename
      mailtoken    
      message
      token
    }
  }
  `



  export interface User {
  id: string;
  firstname: string;
  lastname: string;  
  email: string;
  mobile: string;
  username: string;
  isactivated: number;
  isblocked: number;
  mailtoken: number;
  userpic: string;
  qrcodeurl: string;
}

export interface LoginUserData {
  signinUser: User;
}

export interface LoginUserVariables {
  input: {
    username: string,
    password: string
  }
}
