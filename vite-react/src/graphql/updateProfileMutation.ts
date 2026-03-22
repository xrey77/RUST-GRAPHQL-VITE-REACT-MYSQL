import { gql } from '@apollo/client';

  // mutation UpdateProfile(
  // $id: Int!,
  // $firstName: String!,
  // $lastName: String!,
  // $mobile: String!) {    
  //   updateMutation(
  //     id: $id,
  //     firstName: $firstName,
  //     lastName: $lastName,
  //     mobile: $mobile      
  //   ) {
  //   message
  //   }
  // }

export const UPDATE_PROFILE = gql`
  mutation UpdateProfile(
    $id: ID!,
    $firstname: String!,
    $lastname: String!,
    $mobile: String!) {
      updateProfile(
        input: {
          id: $id,
          firstname: $firstname,
          lastname: $lastname,
          mobile: $mobile
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

export interface ProfiledData {
  updateProfile: UserData;
}

export interface ProfileVariables {
    id: number;
    firstname: string
    lastname: string;
    mobile: string;
}
