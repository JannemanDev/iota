// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { type SuiObjectResponse } from '@mysten/sui.js/client';

import { useResolveVideo } from '~/hooks/useResolveVideo';
import { ObjectDetails } from '~/ui/ObjectDetails';
import { parseObjectType } from '~/utils/objectUtils';
import { trimStdLibPrefix } from '~/utils/stringUtils';

type OwnedObjectProps = {
    obj: SuiObjectResponse;
};

export default function OwnedObject({ obj }: OwnedObjectProps) {
    const video = useResolveVideo(obj);
    const displayMeta = obj.data?.display?.data;

    return (
        <ObjectDetails
            noTypeRender
            variant="small"
            id={obj.data?.objectId}
            type={trimStdLibPrefix(parseObjectType(obj))}
            name={displayMeta?.name ?? displayMeta?.description ?? '--'}
            image={displayMeta?.image_url}
            video={video}
        />
    );
}
