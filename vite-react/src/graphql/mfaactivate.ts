import { gql } from '@apollo/client';

export const ACTIVATE_MFA = gql`
  mutation MfaActivation(
    $id: ID!,
    $twofactorenabled: Boolean!) {
      mfaActivation(
        input: {
          id: $id,
          twofactorenabled: $twofactorenabled
        }
      ) {
        message
        qrcodeurl
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

export interface MfaActivationData {
  mfaActivation: UserData;
}

export interface MfaActivationVariables {
    id: number;
    twofactorenabled: boolean;
}
