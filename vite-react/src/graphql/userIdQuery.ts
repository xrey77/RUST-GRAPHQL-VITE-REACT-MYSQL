import { gql } from '@apollo/client';

export const GETUSERID_QUERY = gql`
  query GetUserId($id: ID!) {
    getUserId(id: $id){
      id
      firstname
      lastname
      email
      mobile
      isactivated
      isblocked
      mailtoken
      userpicture
      qrcodeurl
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
  userpicture: string;
  qrcodeurl?: string;
}

export interface GetUserIdData {
  getUserId: UserData
}

export interface GetUserIdVariables {
  id: number;
}
