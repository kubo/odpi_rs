use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;

#[derive(Debug)]
#[odpic_doc]
pub struct Object {
    pub(crate) handle: *mut dpiObject,
}

impl Object {
    pub(crate) fn new(handle: *mut dpiObject) -> Object {
        Object { handle }
    }

    pub(crate) fn with_add_ref(handle: *mut dpiObject) -> Object {
        unsafe { dpiObject_addRef(handle) };
        Object { handle }
    }

    // // append an element to the collection
    // DPI_EXPORT int dpiObject_appendElement(dpiObject *obj,
    //         dpiNativeTypeNum nativeTypeNum, dpiData *value);

    pub fn copy(&self) -> Result<Object> {
        Ok(Object::new(get_value!(dpiObject_copy(self.handle))?))
    }

    pub fn delete_element_by_index(&self, index: i32) -> Result<()> {
        call!(dpiObject_deleteElementByIndex(self.handle, index))
    }

    // // get the value of the specified attribute
    // DPI_EXPORT int dpiObject_getAttributeValue(dpiObject *obj, dpiObjectAttr *attr,
    //         dpiNativeTypeNum nativeTypeNum, dpiData *value);

    pub fn element_exists_by_index(&self, index: i32) -> Result<bool> {
        Ok(get_value!(dpiObject_getElementExistsByIndex(self.handle, index))? != 0)
    }

    // // get the value of the element in a collection at the specified index
    // DPI_EXPORT int dpiObject_getElementValueByIndex(dpiObject *obj, int32_t index,
    //         dpiNativeTypeNum nativeTypeNum, dpiData *value);

    pub fn first_index(&self) -> Result<Option<i32>> {
        let (index, exists) = get_2values!(dpiObject_getFirstIndex(self.handle))?;
        if exists != 0 {
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }

    pub fn last_index(&self) -> Result<Option<i32>> {
        let (index, exists) = get_2values!(dpiObject_getLastIndex(self.handle))?;
        if exists != 0 {
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }

    pub fn next_index(&self, index: i32) -> Result<Option<i32>> {
        let (index, exists) = get_2values!(dpiObject_getNextIndex(self.handle, index))?;
        if exists != 0 {
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }

    pub fn prev_index(&self, index: i32) -> Result<Option<i32>> {
        let (index, exists) = get_2values!(dpiObject_getPrevIndex(self.handle, index))?;
        if exists != 0 {
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }

    pub fn size(&self) -> Result<i32> {
        get_value!(dpiObject_getSize(self.handle))
    }

    // // set the value of the specified attribute
    // DPI_EXPORT int dpiObject_setAttributeValue(dpiObject *obj, dpiObjectAttr *attr,
    //         dpiNativeTypeNum nativeTypeNum, dpiData *value);

    // // set the value of the element in a collection at the specified index
    // DPI_EXPORT int dpiObject_setElementValueByIndex(dpiObject *obj, int32_t index,
    //         dpiNativeTypeNum nativeTypeNum, dpiData *value);

    pub fn trim(&self, num_to_trim: u32) -> Result<()> {
        call!(dpiObject_trim(self.handle, num_to_trim))
    }
}

impl Clone for Object {
    fn clone(&self) -> Object {
        unsafe { dpiObject_addRef(self.handle) };
        Object {
            handle: self.handle,
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        release_handle!(dpiObject_release(self.handle));
    }
}
