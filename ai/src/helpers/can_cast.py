def can_cast(src_t, dest_t):
    try:
        dest_t(src_t())
        return True
    except:
        return False
