// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { SVGProps } from 'react';

const SvgCoins24 = (props: SVGProps<SVGSVGElement>) => (
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="1em"
		height="1em"
		fill="none"
		viewBox="0 0 24 24"
		{...props}
	>
		<path
			fill="currentColor"
			fillRule="evenodd"
			d="M14 4c-1.508 0-2.904.203-3.95.551-.518.173-.992.395-1.352.679-.35.276-.698.7-.698 1.27v2.75a.75.75 0 0 0 1.5 0V8.238c.175.077.36.147.55.211C11.096 8.797 12.492 9 14 9s2.904-.203 3.95-.551l.058-.02-.737 3.027a.75.75 0 0 0 1.003.876c.434-.17.836-.385 1.142-.657.304-.27.584-.665.584-1.175v-4c0-.57-.347-.994-.698-1.27-.36-.284-.834-.506-1.352-.679C16.904 4.203 15.508 4 14 4Zm0 1.5c.982 0 1.884.094 2.63.25h-5.26c.746-.156 1.648-.25 2.63-.25Zm0 2c-.982 0-1.884-.094-2.63-.25h5.26c-.746.156-1.647.25-2.63.25ZM10 11c-1.508 0-2.904.203-3.95.551-.518.173-.992.395-1.352.679-.35.277-.698.7-.698 1.27v4c0 .57.347.994.698 1.27.36.284.834.506 1.353.679C7.096 19.797 8.492 20 10 20s2.904-.203 3.95-.551c.518-.173.992-.395 1.352-.679.35-.276.698-.7.698-1.27v-4c0-.57-.347-.993-.698-1.27-.36-.284-.834-.506-1.352-.679C12.904 11.203 11.508 11 10 11Zm-4.5 6.467v-2.23c.175.078.36.148.55.212C7.097 15.797 8.493 16 10 16s2.904-.203 3.95-.551c.19-.064.375-.134.55-.211v2.23a.584.584 0 0 1-.127.124c-.171.136-.466.29-.898.434-.855.285-2.083.474-3.475.474s-2.62-.19-3.475-.474c-.431-.144-.726-.299-.898-.434a.583.583 0 0 1-.127-.125Zm1.87-4.717h5.26A13.093 13.093 0 0 0 10 12.5c-.983 0-1.884.094-2.63.25ZM10 14.5c.982 0 1.884-.094 2.63-.25H7.37c.746.156 1.647.25 2.63.25Zm4.507 2.953-.001.005.001-.005Zm-9.014 0 .001.005c-.001-.003-.002-.004-.001-.005Z"
			clipRule="evenodd"
		/>
	</svg>
);
export default SvgCoins24;
