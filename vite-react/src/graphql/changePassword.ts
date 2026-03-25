import { gql } from '@apollo/client';

  // mutation UpdatePassword(
  //   $id: ID!,
  //   $password: String!) {
  //     updatePassword(
  //       input: {
  //         id: $id,
  //         password: $password
  //       }  
  //     ) {
  //       message
  //       errors
  //     }  
  // }

export const CHANGE_PASSWORD = gql`
  mutation ChangePassword($input: ChangePasswordInput!) {
    changePassword(input: $input) {
      message
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
  changePassword: UserData;
}

export interface PasswordVariables {
  input: {
    id: number,
    password: string
  }
}
