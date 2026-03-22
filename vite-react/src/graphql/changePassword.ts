import { gql } from '@apollo/client';

export const CHANGE_PASSWORD = gql`
  mutation UpdatePassword(
    $id: ID!,
    $password: String!) {
      updatePassword(
        input: {
          id: $id,
          password: $password
        }  
      ) {
        message
        errors
      }  
  }
`;


export interface UserData {
  id: number;
  firstname: string;
  lastname: string;
  email: string;
  mobile: string;
  username: string;
  isactivated: boolean;
  isblocked: boolean;
  mailtoken: string;
  userpic: string;
  qrcodeurl: string;
}

export interface PasswordData {
  updatePassword: UserData;
}

export interface PasswordVariables {
    id: number;
    password: string;
}
