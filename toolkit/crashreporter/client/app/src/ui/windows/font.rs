/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use windows_sys::Win32::{Graphics::Gdi, UI::Controls};

/// Windows font handle (`HFONT`).
pub struct Font(Gdi::HFONT);

impl Font {
    /// Get the system theme caption font.
    pub fn caption() -> Self {
        unsafe {
            let mut font = std::mem::zeroed::<Gdi::LOGFONTW>();
            success!(hresult
                Controls::GetThemeSysFont(0, Controls::TMT_CAPTIONFONT as i32, &mut font)
            );
            Font(success!(pointer Gdi::CreateFontIndirectW(&font)))
        }
    }
}

impl std::ops::Deref for Font {
    type Target = Gdi::HFONT;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { Gdi::DeleteObject(self.0 as _) };
    }
}
