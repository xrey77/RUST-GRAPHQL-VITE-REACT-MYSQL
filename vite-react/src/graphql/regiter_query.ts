import { gql } from '@apollo/client';

  // mutation RegisterUser(
  //   $firstName: String!, 
  //   $lastName: String!, 
  //   $email: String!, 
  //   $mobile: String!, 
  //   $username: String!, 
  //   $password: String!
  // ) {
  //   userMutation(
  //     firstName: $firstName, 
  //     lastName: $lastName, 
  //     email: $email, 
  //     mobile: $mobile, 
  //     username: $username, 
  //     password: $password
  //   ) {
  //     message
  //     user {
  //       id
  //       username
  //       email
  //     }
  //   }
  // }


export const SIGNUP_MUTATION = gql`
  mutation RegisterNewUser(
    $firstname: String!, 
    $lastname: String!, 
    $email: String!, 
    $mobile: String,  
    $username: String!, 
    $password: String!
  ) {
    registerUser(
      input: {
        firstname: $firstname,
        lastname: $lastname,
        email: $email,
        mobile: $mobile,      
        username: $username,
        password: $password
      }
    ) {
      message
      errors
    }
  }
  `

interface User {
  id: string;
  firstName: string;
  lastName: string;  
  email: string;
  mobile: string;
  username: string;
  password: string;
}

export interface CreateUserData {
  registerUser: User;
}

export interface CreateUserVariables {
    firstname: string;
    lastname: string;    
    email: string;
    mobile: string;
    username: string;
    password: string;
}
