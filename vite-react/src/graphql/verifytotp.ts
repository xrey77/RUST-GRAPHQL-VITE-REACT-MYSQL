import { gql } from '@apollo/client';

  // mutation VerifyTotp(
  //   $id: ID!,
  //   $otp: String!
  // ) {
  //   verifyTotp(input: {
  //     id: $id,
  //     otp: $otp
  //   }) {
  //     username
  //     message
  //     errors
  //   }
  // }

export const VERIFY_OTP = gql`
  mutation OtpVerification($input: OtpVerificationInput!) {
    otpVerification(input: $input) {
      username
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

export interface OtpVerificationData {
  otpVerification: UserData;
}

export interface OtpVerificationVariables {
  input: {
    id: number,
    otp: string
  }
}
