// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if ENCODER && OPCODE_INFO
namespace Iced.Intel {
	/// <summary>Opcode table</summary>
	public enum OpCodeTableKind {
		/// <summary>Legacy/<c>MAP0</c> table</summary>
		Normal = 0,
		/// <summary><c>0F</c>/<c>MAP1</c> table (legacy, VEX, EVEX, MVEX)</summary>
		T0F = 1,
		/// <summary><c>0F38</c>/<c>MAP2</c> table (legacy, VEX, EVEX, MVEX)</summary>
		T0F38 = 2,
		/// <summary><c>0F3A</c>/<c>MAP3</c> table (legacy, VEX, EVEX, MVEX)</summary>
		T0F3A = 3,
		/// <summary><c>MAP5</c> table (EVEX)</summary>
		MAP5 = 4,
		/// <summary><c>MAP6</c> table (EVEX)</summary>
		MAP6 = 5,
		/// <summary><c>MAP8</c> table (XOP)</summary>
		MAP8 = 6,
		/// <summary><c>MAP9</c> table (XOP)</summary>
		MAP9 = 7,
		/// <summary><c>MAP10</c> table (XOP)</summary>
		MAP10 = 8,
	}
}
#endif
