import poseidon
from constants import round_constants, matrix

# poseidon_simple, t = poseidon.parameters.case_simple()
# input_vec = [x for x in range(0, t)]
# print("Input: ", input_vec)
# poseidon_digest = poseidon_simple.run_hash(input_vec)
# print("Output: ", hex(int(poseidon_digest)))

security_level = 128
input_rate = 2
t = 3
full_round = 8
partial_round = 56
alpha = 5
# modulus = poseidon.parameters.prime_254
modulus = 0x40000000000000000000000000000000224698fc094cf91b992d30ed00000001  # pallas


poseidon_custom = poseidon.OptimizedPoseidon(poseidon.HashType.CONSTINPUTLEN, modulus, security_level, alpha, input_rate, t, full_round=full_round,
                                             partial_round=partial_round, rc_list=round_constants, mds_matrix=matrix)
# poseidon_custom = poseidon.Poseidon(modulus, security_level, alpha, input_rate, t, full_round=full_round,
#                                     partial_round=partial_round, rc_list=round_constants, mds_matrix=matrix)
print(poseidon_custom.run_hash([1, 2]))
