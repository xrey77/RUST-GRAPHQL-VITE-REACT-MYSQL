import { gql } from '@apollo/client';

export const UPLOAD_PICTURE = gql`
  mutation UploadPicture($id: Int!, $file: Upload!) {
    uploadPicture(
      input: {
        id: $id,
        file: $file
      }
    ) {
      userpic
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
  userpicture: string;
  qrcodeurl: string;
}

export interface UploadData {
  uploadPicture: UserData;
}

export interface UploadVariables {
    id: number,
    file: File
}
