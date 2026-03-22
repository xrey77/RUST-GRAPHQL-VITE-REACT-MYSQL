import { gql } from '@apollo/client';

  // mutation UserLogin($username: String!, $password: String!) {
  //   loginMutation(username: $username, password: $password) {
  //     user {
  //       id
  //       firstName
  //       lastName      
  //       email
  //       mobile
  //       username
  //       isActivated
  //       isBlocked
  //       userpicture
  //       mailtoken
  //       qrcodeurl
  //     }
  //     token
  //     message


export const SIGNIN_MUTATION = gql`
    mutation LoginUser(
      $username: String!,
      $password: String!
    ) {
      loginUser(
        input:{
          username: $username,
          password: $password
        }) 
      {
        user {
          id
          firstname,
          lastname,
          email
          mobile
          username
          isactivated
          isblocked
          mailtoken
          userpicture
          qrcodeurl    
        }  
        token
        message 
      	errors
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
  loginUser: User;
}

export interface LoginUserVariables {
    username: string;
    password: string;
}
