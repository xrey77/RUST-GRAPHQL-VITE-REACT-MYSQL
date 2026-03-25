import { gql } from '@apollo/client';

  // mutation MfaActivation(
  //   $id: ID!,
  //   $twofactorenabled: Boolean!) {
  //     mfaActivation(
  //       input: {
  //         id: $id,
  //         twofactorenabled: $twofactorenabled
  //       }
  //     ) {
  //       message
  //       qrcodeurl
  //       errors
  //     }
  // }

export const ACTIVATE_MFA = gql`
  mutation ActivateMfa($input: ActivateMfaInput!) {
    activateMfa(input: $input) {
      qrcodeurl
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

export interface MfaActivationData {
  activateMfa: UserData;
}

export interface MfaActivationVariables {
  input: {
    id: number,
    twofactorenabled: boolean
  }
}
