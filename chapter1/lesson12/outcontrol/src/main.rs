mod outer {
    // Видно только в этом крейте (не экспортируется в зависимости)
    pub(crate) fn crate_visible() {}
    
    // Видно только в родительском модуле
    pub(super) fn parent_visible() {}
    
    // Видно только в подмодуле `inner`
    pub(in crate::outer) fn restricted() {}
    
    mod inner {
        fn access() {
            crate_visible();  // OK
            parent_visible(); // OK
            restricted();     // OK
        }
    }
}
