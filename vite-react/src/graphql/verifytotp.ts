import { gql } from '@apollo/client';

  // mutation VerifyTotp($id: Int!, $otp: String!) {
  //   verifyTotp(id: $id, otp: $otp) {
  //     username
  //     message
  //   }
  // }

export const VERIFY_OTP = gql`
  mutation VerifyTotp(
    $id: ID!,
    $otp: String!
  ) {
    verifyTotp(input: {
      id: $id,
      otp: $otp
    }) {
      username
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

export interface OtpVerificationData {
  verifyTotp: UserData;
}

export interface OtpVerificationVariables {
    id: number;
    otp: string;
}
