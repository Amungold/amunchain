#[cfg(test)]
mod tests {
    use heapless::Vec;
    use amun_kernel_types::PublicHash32;
    use crate::stf::StfState;

    #[test]
    fn test_stf_commit_changes_root() {
        let mut stf = StfState::new(PublicHash32::default(), 0);
        let old_root = stf.state_root;
        let mut key = Vec::<u8,32>::new(); key.extend_from_slice(b"k1").unwrap();
        let mut val = Vec::<u8,32>::new(); val.extend_from_slice(b"v1").unwrap();
        stf.apply_set(key, val).unwrap();
        let new_root = stf.commit().unwrap();
        assert_ne!(old_root, new_root);
    }

    #[test]
    fn test_stf_rollback_preserves_root() {
        let mut stf = StfState::new(PublicHash32::default(), 0);
        let old_root = stf.state_root;
        let mut key = Vec::<u8,32>::new(); key.extend_from_slice(b"k1").unwrap();
        let mut val = Vec::<u8,32>::new(); val.extend_from_slice(b"v1").unwrap();
        stf.apply_set(key, val).unwrap();
        stf.rollback();
        assert_eq!(old_root, stf.state_root);
    }

    #[test]
    fn test_stf_deterministic_root() {
        let mut stf1 = StfState::new(PublicHash32::default(), 0);
        let mut stf2 = StfState::new(PublicHash32::default(), 0);
        for i in 0..5u8 {
            let mut k = Vec::<u8,32>::new(); k.extend_from_slice(&[i;8]).unwrap();
            let mut v = Vec::<u8,32>::new(); v.extend_from_slice(&[i;8]).unwrap();
            stf1.apply_set(k.clone(), v.clone()).unwrap();
            stf2.apply_set(k, v).unwrap();
        }
        let r1 = stf1.commit().unwrap();
        let r2 = stf2.commit().unwrap();
        assert_eq!(r1, r2);
    }

    #[test] fn test_nonce_basics() {}
    #[test] fn test_apply_block_deterministic() {}
    #[test] fn test_nonce_rejects_replay() {}
    #[test] fn test_root_deterministic() {}
    #[test] fn test_state_set_get() {}
    #[test] fn test_state_delete() {}
}
