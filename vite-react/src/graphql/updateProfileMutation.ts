import { gql } from '@apollo/client';

export const UPDATE_PROFILE = gql`
  mutation UpdateProfile($input: ProfileInput!) {
    profileUpdate(input: $input){
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

export interface ProfiledData {
  profileUpdate: UserData;
}

export interface ProfileVariables {
  input: {
    id: number,
    firstname: string,
    lastname: string,
    mobile: string
  }
}
